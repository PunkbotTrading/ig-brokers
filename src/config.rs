#[derive(Clone)]
pub struct Config {
	pub base_url: String
}

impl Config {
	pub fn live() -> Config {
		Config {
			base_url: String::from("api.ig.com/gateway/deal")
		}
	}

	pub fn demo() -> Config {
		Config {
			base_url: String::from("demo-api.ig.com/gateway/deal")
		}
	}
}
