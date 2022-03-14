use serde::Deserialize;

// TODO consider use env_var/cli_arg override config
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub api_server: ApiServer,
    pub postgres: Postgres,
}

impl Config {
    pub fn read_config_file(path: std::path::PathBuf) -> Self {
        let config = std::fs::read_to_string(path).unwrap();
        toml::de::from_str(&config).unwrap()
    }
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ApiServer {
    port: u16,
}

impl ApiServer {
    pub fn listen_addr(&self) -> std::net::SocketAddr {
        std::net::SocketAddr::V4(std::net::SocketAddrV4::new(
            std::net::Ipv4Addr::UNSPECIFIED,
            self.port,
        ))
    }
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Postgres {
    host: String,
    port: u16,
    username: String,
    password: String,
    dbname: String,
}

impl Postgres {
    pub fn db_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.dbname
        )
    }
}
