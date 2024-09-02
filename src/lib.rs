mod client;
pub mod config;
pub mod api;
pub mod models;

/*
#[cfg(test)]
mod tests {
    use crate::api::IG;
    use crate::models::*;
    use chrono::Utc;
    use dotenv::dotenv;
    use std::env;

    fn setup() -> (String, String, String, String) {
        dotenv().unwrap();
        let account_id = env::var("IG_ACCOUNT_ID").unwrap();
        let api_key = env::var("IG_API_KEY").unwrap();
        let username = env::var("IG_USERNAME").unwrap();
        let password = env::var("IG_PASSWORD").unwrap();
        (account_id, api_key, username, password)
    }

    fn get_api() -> IG {
        let (account_id, api_key, username, password) = setup();
        IG::demo(account_id, api_key, username, password)
    }

    /*
    #[test]
    fn get_accounts() {
        let api = get_api();
        let res = api.get_accounts();
        assert_eq!(res.await.is_ok(), true);
    }

    #[test]
    fn get_preferences() {
        let api = get_api();
        let res = api.get_preferences();
        assert_eq!(res.is_ok(), true);
    }

    #[test]
    fn update_preferences() {
        let api = get_api();
        let pref = Preferences {
            trailing_stops_enabled: true,
        };
        let res = api.update_preferences(&pref);
        assert_eq!(res.is_ok(), true);
    }

    #[test]
    fn get_activity_history_1() {
        let api = get_api();
        let mut params = ActivityHistoryQuery::default();
        params.from = Utc::now().naive_local().into();
        let res = api.get_activity_history(&params);
        assert_eq!(res.is_ok(), true);
    }

    #[test]
    fn get_activity_history_2() {
        let api = get_api();
        let mut query = ActivityHistoryQuery::default();
        query.from = (Utc::now() - chrono::Duration::days(365))
            .naive_local()
            .into();
        let res = api.get_activity_history(&query);
        assert_eq!(res.is_ok(), true);
    }

    #[test]
    fn get_transaction_history_1() {
        let api = get_api();
        let mut query = TransactionHistoryQuery::default();
        query.from = Utc::now().naive_local().into();
        let res = api.get_transaction_history(&query);
        assert_eq!(res.is_ok(), true);
    }

    #[test]
    fn get_transaction_history_2() {
        let api = get_api();
        let mut query = TransactionHistoryQuery::default();
        query.from = (Utc::now() - chrono::Duration::days(365))
            .naive_local()
            .into();
        let res = api.get_transaction_history(&query);
        assert_eq!(res.is_ok(), true);
    }

    #[test]
    fn get_client_sentiments() {
        let api = get_api();
        let mut query = SentimentQuery::default();
        query.market_ids = Some(vec!["VOD-UK".to_string()]);
        let res = api.get_client_sentiments(&query);
        assert_eq!(res.is_ok(), true);
    }

    #[test]
    fn get_client_sentiment() {
        let api = get_api();
        let res = api.get_client_sentiment(&"VOD-UK".into());
        assert_eq!(res.is_ok(), true);
    }

    #[test]
    fn get_related_client_sentiment() {
        let api = get_api();
        let res = api.get_related_client_sentiment(&"VOD-UK".into());
        assert_eq!(res.is_ok(), true);
    }

    #[test]
    fn get_deal_confirmation() {
        let api = get_api();

        // Get positions
        let positions = api.get_positions().expect("failed to get positions");
        let first = positions.positions.first().expect("no positions found");

        // Get deal confirmation
        let res = api.get_deal_confirmation(&first.position.deal_reference);
        assert_eq!(res.is_ok(), true);
    }

    #[test]
    fn get_positions() {
        let api = get_api();
        let res = api.get_positions();
        assert_eq!(res.is_ok(), true);
    }

    #[test]
    fn get_position() {
        let api = get_api();

        // Get positions
        let positions = api.get_positions().expect("failed to get positions");
        let first = positions.positions.first().expect("no positions found");

        // Get deal confirmation
        let res = api.get_position(&first.position.deal_id);
        assert_eq!(res.is_ok(), true);
    }

    #[test]
    fn close_position() {
        let api = get_api();

        let positions = api.get_positions().expect("failed to get positions");
        let first = positions.positions.first().expect("no positions found");

        let mut position = ClosePosition::default();
        position.deal_id = first.position.deal_id.clone().into();
        position.direction = Direction::Sell.into();
        position.order_type = OrderType::Market.into();
        position.size = 1.0.into();

        let res = api.close_position(&position);
        assert_eq!(res.is_ok(), true);
    }

    #[test]
    fn create_position() {
        let api = get_api();

        let mut position = CreatePosition::default();
        position.currency_code = "AUD".to_string().into();
        position.direction = Direction::Buy.into();
        position.epic = "CS.D.BITCOIN.CFD.IP".to_string().into();
        position.expiry = "-".to_string().into();
        position.force_open = false.into();
        position.guaranteed_stop = false.into();
        position.order_type = OrderType::Market.into();
        position.size = 1.0.into();

        let res = api.create_position(&position);
        assert_eq!(res.is_ok(), true);
    }

    #[test]
    fn update_position() {
        // TODO: design test
    }

    #[test]
    fn get_sprint_market_positions() {
        let api = get_api();
        let res = api.get_sprint_market_positions();
        assert_eq!(res.is_ok(), true);
    }

    #[test]
    fn create_sprint_market_position() {
        let api = get_api();
        
        let mut position = CreateSprintMarketPosition::default();
        position.direction = Direction::Buy.into();
        position.epic = "CS.D.BITCOIN.CFD.IP".to_string().into();
        position.expiry_period = SprintMarketExpiryPeriod::SixtyMinutes.into();
        position.size = 1.0.into();

        let res = api.create_sprint_market_position(&position);
        assert_eq!(res.is_ok(), true);
    }

    #[test]
    fn get_working_orders() {
        let api = get_api();
        let res = api.get_working_orders();
        assert_eq!(res.is_ok(), true);
    }

    #[test]
    fn create_working_order() {
        // TODO: design test
    }

    #[test]
    fn delete_working_order() {
        // TODO: design test
    }

    #[test]
    fn update_working_order() {
        // TODO: design test
    }

    #[test]
    fn get_applications() {
        let api = get_api();
        let res = api.get_applications();
        assert_eq!(res.is_ok(), true);
    }

    #[test]
    fn update_application() {
        let api = get_api();
        let req = UpdateApplication {
            allowance_account_overall: 10.0,
            allowance_account_trading: 33.0,
            api_key: env::var("IG_API_KEY").unwrap(),
            status: ApplicationStatus::Disabled
        };

        let res = api.update_application(&req);
        assert_eq!(res.is_ok(), true);
    }

    #[test]
    fn get_session() {
        let api = get_api();
        let res = api.get_session();
        assert_eq!(res.is_ok(), true);
    }

    #[test]
    fn get_market_categories() {
        let api = get_api();
        let res = api.get_market_categories();
        assert_eq!(res.is_ok(), true);
    }

    #[test]
    fn get_market_category() {
        let api = get_api();
        let res = api.get_market_categories().expect("market call failed");
        let nodes = res.nodes.expect("nodes was null");
        let crypto = nodes
            .iter()
            .find(|n| n.name == "Cryptocurrency".to_string());
        let crypto = crypto.expect("crypto not found");

        let res = api.get_market_category(&crypto.id);
        assert_eq!(res.is_ok(), true);
    }

    #[test]
    fn get_markets() {
        let api = get_api();
        let mut query = MarketsQuery::default();
        query.epics.push("CS.D.BITCOIN.CFD.IP".to_string());
        let res = api.get_markets(&query);
        assert_eq!(res.is_ok(), true);
    }

    #[test]
    fn search_markets() {
        let api = get_api();
        let res = api.search_markets(&"eth".into());
        assert_eq!(res.is_ok(), true);
    }

    #[test]
    fn get_prices() {
        let api = get_api();
        let query = PricesQuery::default();
        let res = api.get_prices(&"CS.D.BITCOIN.CFD.IP".to_string(), &query);    
        assert_eq!(res.is_ok(), true);
    }

    #[test]
    fn get_watchlists() {
        let api = get_api();
        let res = api.get_watchlists();
        assert_eq!(res.is_ok(), true);
    }

    #[test]
    fn create_watchlist() {
        let api = get_api();
        let epics = vec!["CS.D.BITCOIN.CFD.IP".to_string()];
        let name = "test".to_string();
        let create = CreateWatchlist { epics, name };
        let create_res = api.create_watchlist(&create);
        assert_eq!(create_res.is_ok(), true);
    }

    #[test]
    fn get_watchlist() {
        let api = get_api();

        let res = api.get_watchlists().unwrap();
        let watchlist = res.watchlists.iter()
            .find(|w| w.name == "test")
            .expect("no watchlist named test");

        let wres = api.get_watchlist(&watchlist.id);
        assert_eq!(wres.is_ok(), true);
    }

    #[test]
    fn add_market_to_watchlist() {
        let api = get_api();

        let res = api.get_watchlists().unwrap();
        let watchlist = res.watchlists.iter()
            .find(|w| w.name == "test")
            .expect("no watchlist named test");

        let req = AddToWatchlist { epic: "CS.D.ETHUSD.CFD.IP".to_string() };
        let res2 = api.add_market_to_watchlist(&watchlist.id, &req);
        assert_eq!(res2.is_ok(), true);
    }

    #[test]
    fn remove_market_from_watchlist() {
        let api = get_api();

        let res = api.get_watchlists().unwrap();
        let watchlist = res.watchlists.iter()
            .find(|w| w.name == "test")
            .expect("no watchlist named test");

        let res2 = api.remove_market_from_watchlist(&watchlist.id, &"CS.D.ETHUSD.CFD.IP".to_string());
        assert_eq!(res2.is_ok(), true);
    }

    #[test]
    fn delete_watchlist() {
        let api = get_api();

        let res = api.get_watchlists().unwrap();
        let watchlist = res.watchlists.iter()
            .find(|w| w.name == "test")
            .expect("no watchlist named test");

        let del_res = api.delete_watchlist(&watchlist.id);
        assert_eq!(del_res.is_ok(), true);
    }
    
     */
}

 */
