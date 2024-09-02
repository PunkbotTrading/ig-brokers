
use crate::client::Client;
use crate::config::Config;
use crate::models::*;
use reqwest::Error;

#[derive(Clone)]
pub struct IG {
	client: Client
}

impl IG {
	/// Creates a new API instance with the production configuration.
	pub fn live(account_id: String, api_key: String, username: String, password: String) -> IG {
		IG::new(account_id, api_key, username, password, Config::live())
	}

	/// Creates a new API instnace with the demo configuration.
	pub fn demo(account_id: String, api_key: String, username: String, password: String) -> IG {
		IG::new(account_id, api_key, username, password, Config::demo())
	}

	/// Creates a new API instance with a config.
	pub fn new(account_id: String, api_key: String, username: String, password: String, config: Config) -> IG {
		let client = Client::new(account_id, api_key, username, password, config);
		IG { client }
	}

	/// GET /accounts
	/// Returns a list of accounts belonging to the logged-in client.
	pub async fn get_accounts(&self) -> Result<Accounts, Error> {
		let endpoint: String = "/accounts".into();
		let data: Accounts = self.client.get_signed(&endpoint, 1, None::<()>).await?;
		Ok(data)
	}

	/// GET /accounts/preferences
	/// Returns account preferences.
	pub async fn get_preferences(&self) -> Result<Preferences, Error> {
		let endpoint: String = "/accounts/preferences".into();
		let data: Preferences = self.client.get_signed(&endpoint, 1, None::<()>).await?;
		Ok(data)
	}

	/// PUT /accounts/preferences
	/// Updates account preferences.
	pub async fn update_preferences(&self, preferences: &Preferences) -> Result<OkResponse, Error> {
		let endpoint: String = "/accounts/preferences".into();
		let data: OkResponse = self.client.put_signed(&endpoint, 1, Some(preferences)).await?;
		Ok(data)
	}

	/// GET /history/activity
	/// Returns the account activity history.
	pub async fn get_activity_history(&self, query: &ActivityHistoryQuery) -> Result<ActivityHistory, Error> {
		let endpoint: String = "/history/activity".into();
		let data: ActivityHistory = self.client.get_signed(&endpoint, 3, Some(query)).await?;
		Ok(data)
	}

	/// GET /history/transactions
	/// Returns the transaction history.
	/// By default returns the minute prices within the last 10 minutes.
	pub async fn get_transaction_history(&self, query: &TransactionHistoryQuery) -> Result<TransactionHistory, Error> {
		let endpoint: String = "/history/transactions".into();
		let data: TransactionHistory = self.client.get_signed(&endpoint, 2, Some(query)).await?;
		Ok(data)
	}

	/// GET /clientsentiment
	/// Returns the client sentiment for the given instrument's market.
	pub async fn get_client_sentiments(&self, query: &SentimentQuery) -> Result<Sentiments, Error> {		
		let endpoint = "/clientsentiment".to_string();
		let data: Sentiments = self.client.get_signed(&endpoint, 1, Some(query)).await?;
		Ok(data)
	}

	/// GET /clientsentiment/{marketId}
	/// Returns the client sentiment for the given instrument's market.
	pub async fn get_client_sentiment(&self, market_id: &String) -> Result<Sentiment, Error> {
		let endpoint = format!("/clientsentiment/{}", market_id);
		let data: Sentiment = self.client.get_signed(&endpoint, 1, None::<()>).await?;
		Ok(data)
	}

	/// GET /clientsentiment/related/{marketId}
	/// Returns a list of related (what others have traded) client sentiment for the given instrument's market.
	pub async fn get_related_client_sentiment(&self, market_id: &String) -> Result<Sentiments, Error> {
		let endpoint = format!("/clientsentiment/related/{}", market_id);
		let data: Sentiments = self.client.get_signed(&endpoint, 1, None::<()>).await?;
		Ok(data)
	}

	/// GET /confirms/{dealReference}
	/// Returns a deal confirmation for the given deal reference.
	pub async fn get_deal_confirmation(&self, deal_reference: &String) -> Result<DealConfirmation, Error> {
		let endpoint = format!("/confirms/{}", deal_reference);
		let data: DealConfirmation = self.client.get_signed(&endpoint, 1, None::<()>).await?;
		Ok(data)
	}

	/// GET /positions
	/// Returns all open positions for the active account.
	pub async fn get_positions(&self) -> Result<Positions, Error> {
		let endpoint = "/positions".to_string();
		let data: Positions = self.client.get_signed(&endpoint, 2, None::<()>).await?;
		Ok(data)
	}

	/// GET /positions/{dealId}
	/// Returns an open position for the active account by deal identifier.
	pub async fn get_position(&self, deal_id: &String) -> Result<Position, Error> {
		let endpoint = format!("/positions/{}", deal_id);
		let data: Position = self.client.get_signed(&endpoint, 2, None::<()>).await?;
		Ok(data)
	}

	/// DELETE /positions/otc
	/// Closes one or more OTC positions.
	pub async fn close_position(&self, req: &ClosePosition) -> Result<DealRef, Error> {
		let endpoint: String = "/positions/otc".into();
		let data: DealRef = self.client.delete_signed(&endpoint, 1, Some(req)).await?;
		Ok(data)
	}

	/// POST /positions/otc
	/// Creates an OTC position.
	pub async fn create_position(&self, req: &CreatePosition) -> Result<DealRef, Error> {
		let endpoint: String = "/positions/otc".into();
		let data: DealRef = self.client.post_signed(&endpoint, 2, Some(req)).await?;
		Ok(data)
	}

	/// PUT /positions/otc/{dealId}
	/// Updates an OTC position.
	pub async fn update_position(&self, deal_id: &String, req: &UpdatePosition) -> Result<DealRef, Error> {
		let endpoint = format!("/positions/otc/{}", deal_id);
		let data: DealRef = self.client.put_signed(&endpoint, 2, Some(req)).await?;
		Ok(data)
	}

	/// GET /positions/sprintmarkets
	/// A list of sprint market positions.
	pub async fn get_sprint_market_positions(&self) -> Result<SprintMarketPositions, Error> {
		let endpoint = "/positions/sprintmarkets".to_string();
		let data: SprintMarketPositions = self.client.get_signed(&endpoint, 2, None::<()>).await?;
		Ok(data)
	}

	/// POST /positions/sprintmarkets
	/// Creates a sprint market position.
	pub async fn create_sprint_market_position(&self, req: &CreateSprintMarketPosition) -> Result<DealRef, Error> {
		let endpoint = "/positions/sprintmarkets".to_string();
		let data: DealRef = self.client.post_signed(&endpoint, 1, Some(req)).await?;
		Ok(data)
	}

	/// GET /workingorders
	/// Returns all open working orders for the active account.
	pub async fn get_working_orders(&self) -> Result<WorkingOrders, Error> {
		let endpoint = "/workingorders".to_string();
		let data: WorkingOrders = self.client.get_signed(&endpoint, 2, None::<()>).await?;
		Ok(data)
	}

	/// POST /workingorders/otc
	/// Creates an OTC working order.
	pub async fn create_working_order(&self, req: &CreateWorkingOrder) -> Result<DealRef, Error> {
		let endpoint = "/workingorders/otc".to_string();
		let data: DealRef = self.client.post_signed(&endpoint, 2, Some(req)).await?;
		Ok(data)
	}

	/// DELETE /workingorders/otc/{dealId}
	/// Deletes an OTC working order.
	pub async fn delete_working_order(&self, deal_id: &String) -> Result<DealRef, Error> {
		let endpoint = format!("/workingorders/otc/{}", deal_id);
		let data: DealRef = self.client.delete_signed(&endpoint, 2, None::<()>).await?;
		Ok(data)
	}

	/// PUT /workingorders/otc/{dealId}
	/// Updates an OTC working order.
	pub async fn update_working_order(&self, deal_id: &String, req: &UpdateWorkingOrder) -> Result<DealRef, Error> {
		let endpoint = format!("/workingorders/otc/{}", deal_id);
		let data: DealRef = self.client.put_signed(&endpoint, 2, Some(req)).await?;
		Ok(data)
	}

	/// GET /operations/application
	/// Returns a list of client owned applications.
	pub async fn get_applications(&self) -> Result<Vec<Application>, Error> {
		let endpoint = "/operations/application".to_string();
		let data: Vec<Application> = self.client.get_signed(&endpoint, 1, None::<()>).await?;
		Ok(data)
	}

	/// PUT /operations/application
	/// Alters the details of a given user application.
	pub async fn update_application(&self, req: &UpdateApplication) -> Result<Application, Error> {
		let endpoint = "/operations/application".to_string();
		let data: Application = self.client.put_signed(&endpoint, 1, Some(req)).await?;
		Ok(data)
	}

	/// GET /session
	/// Returns the user's session details and optionally tokens.
	pub async fn get_session(&self) -> Result<Session, Error> {
		let endpoint = "/session".to_string();
		let data: Session = self.client.get_signed(&endpoint, 1, None::<()>).await?;
		Ok(data)
	}

	/// GET /marketnavigation
	/// Returns all top-level nodes (market categories) in the market navigation hierarchy.
	pub async fn get_market_categories(&self) -> Result<MarketCategory, Error> {
		let endpoint: String = "/marketnavigation".into();
		let data: MarketCategory = self.client.get_signed(&endpoint, 1, None::<()>).await?;
		Ok(data)
	}

	/// GET /marketnavigation/{nodeId}
	/// Returns all sub-nodes of the given node in the market navigation hierarchy.
	pub async fn get_market_category(&self, node_id: &String) -> Result<MarketCategory, Error> {
		let endpoint = format!("/marketnavigation/{}", node_id);
		let data: MarketCategory = self.client.get_signed(&endpoint, 1, None::<()>).await?;
		Ok(data)
	}

	/// GET /markets
	/// Returns the details of the given markets.
	pub async fn get_markets(&self, query: &MarketsQuery) -> Result<Markets, Error> {
		let endpoint: String = "/markets".into();
		let data: Markets = self.client.get_signed(&endpoint, 2, Some(query)).await?;
		Ok(data)
	}

	/// GET /markets/{epic}
	/// Returns the details of the given market.
	pub async fn get_market(&self, epic: &String) -> Result<Market, Error> {
		let endpoint = format!("/markets/{}", epic);
		let data: Market = self.client.get_signed(&endpoint, 3, None::<()>).await?;
		Ok(data)
	}

	/// GET /markets?searchTerm={searchTerm}
	/// Returns all markets matching the search term.
	pub async fn search_markets(&self, search_term: &String) -> Result<MarketSearch, Error> {
		let endpoint = format!("/markets?searchTerm={}", search_term);
		let data: MarketSearch = self.client.get_signed(&endpoint, 1, None::<()>).await?;
		Ok(data)
	}

	/// GET /prices/{epic}
	/// Returns historical prices for a particular instrument.
	/// By default returns the minute prices within the last 10 minutes.
	pub async fn get_prices(&self, epic: &String, query: &PricesQuery) -> Result<Prices, Error> {
		let endpoint = format!("/prices/{}", epic);
		let data: Prices = self.client.get_signed(&endpoint, 3, Some(query)).await?;
		Ok(data)
	}

	/// GET /watchlists
	/// Returns all watchlists belonging to the active account
	pub async fn get_watchlists(&self) -> Result<Watchlists, Error> {
		let endpoint = "/watchlists".to_string();
		let data: Watchlists = self.client.get_signed(&endpoint, 1, None::<()>).await?;
		Ok(data)
	}

	/// POST /watchlists
	/// Creates a watchlist.
	pub async fn create_watchlist(&self, req: &CreateWatchlist) -> Result<CreateWatchlistResult, Error> {
		let endpoint = "/watchlists".to_string();
		let data: CreateWatchlistResult = self.client.post_signed(&endpoint, 1, Some(req)).await?;
		Ok(data)
	}

	/// DELETE /watchlists/{watchlistId}
	/// Deletes a watchlist.
	pub async fn delete_watchlist(&self, watchlist_id: &String) -> Result<OkResponse, Error> {
		let endpoint = format!("/watchlists/{}", watchlist_id);
		let data: OkResponse = self.client.delete_signed(&endpoint, 1, None::<()>).await?;
		Ok(data)
	}

	/// GET /watchlists/{watchlistId}
	/// Returns a watchlist.
	pub async fn get_watchlist(&self, watchlist_id: &String) -> Result<MarketSearch, Error> {
		let endpoint = format!("/watchlists/{}", watchlist_id);
		let data: MarketSearch = self.client.get_signed(&endpoint, 1, None::<()>).await?;
		Ok(data)
	}

	/// PUT /watchlists/{watchlistId}
	/// Add a market to watchlist.
	pub async fn add_market_to_watchlist(&self, watchlist_id: &String, req: &AddToWatchlist) -> Result<OkResponse, Error> {
		let endpoint = format!("/watchlists/{}", watchlist_id);
		let data: OkResponse = self.client.put_signed(&endpoint, 1, Some(req)).await?;
		Ok(data)
	}

	/// DELETE /watchlists/{watchlistId}/{epic}
	/// Remove a market from a watchlist.
	pub async fn remove_market_from_watchlist(&self, watchlist_id: &String, epic: &String) -> Result<OkResponse, Error> {
		let endpoint = format!("/watchlists/{}/{}", watchlist_id, epic);
		let data: OkResponse = self.client.delete_signed(&endpoint, 1, None::<()>).await?;
		Ok(data)
	}
}
