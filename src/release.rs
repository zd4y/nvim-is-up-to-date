use serde::Deserialize;
use reqwest::{blocking::Client, header::USER_AGENT};

const API_URL: &str = "https://api.github.com/repos/neovim/neovim/releases/tags/nightly?per_page=1";

#[derive(Deserialize)]
pub struct Release {
    pub name: String,
}

pub fn get_latest() -> Release {
    Client::new()
        .get(API_URL)
        .header(USER_AGENT, "zd4y")
        .send()
        .expect("Error fetching api")
        .json::<Release>()
        .expect("Error parsing json")
}
