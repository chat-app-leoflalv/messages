use dotenv::dotenv;
use std::{env, u16};

pub struct Envs {
    pub port: u16,
    pub server_url: String,
}

impl Envs {
    pub fn new() -> Self {
        dotenv().ok();

        let port_str = env::var("PORT").expect("PORT must be set in .env");
        let port: u16 = port_str.parse().expect("PORT must be a valid number");

        let nats_servers = env::var("NATS_SERVERS").expect("Nats server url must be set in .env");
        let server_url: String = nats_servers
            .split(',')
            .map(|s| s.trim().to_string())
            .next()
            .expect("Incorrect format for nats server");

        Envs { port, server_url }
    }
}
