use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub bootstrap_nodes: Option<Vec<String>>,
    pub server_list_file: Option<String>,
}
