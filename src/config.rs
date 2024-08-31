use clap::{arg, Arg, Args, Command};
use crate::types::Network;
use std::net::SocketAddr;

use clap::builder::TypedValueParser as _;
use clap::Parser;


#[derive(Debug, Clone, Parser)]
pub struct Config {
    // for bitcoin spv
    #[arg(
        long,
        default_value_t = Network::Testnet,
        value_parser = clap::builder::PossibleValuesParser::new(["bitcoin","testnet","testnet4","fractal","regtest","signet"])
            .map(|s| s.parse::<Network>().unwrap()),
    )]
    pub network_type: Network,
    #[arg(long, help = "bitcond rpc address" )]
    pub daemon_rpc_addr: SocketAddr,
    #[arg(long)]
    pub cookie: Option<String>,
    // http port to listen
    #[arg(long)]
    pub http_addr: SocketAddr,
    // for registeration to bool
    #[arg(long, default_value_t = String::from("http://127.0.0.1:9933"))]
    pub subclient_url: String,
    #[arg(long)]
    pub device_owner: String,
    #[arg(long)]
    pub watcher_device_id: String,
    #[arg(long, default_value_t = String::from("0x1234"))]
    pub relate_device_id_test: String,
    #[arg(long, default_value_t = false)]
    pub sgx_enable: bool,
}

impl Config {
    pub fn from_args() -> Config {
        let args = Config::parse();
        args
    }
}