#[macro_use]
extern crate log;

mod server;

use structopt::StructOpt;
use anyhow::Result;
use futures::future;
use futures::StreamExt;
use std::net::SocketAddr;
use rusty_workers::tarpc::{self, server::Channel};
use rusty_workers::rpc::RuntimeService;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "rusty-workers-runtime",
    about = "Rusty Workers (runtime)"
)]
struct Opt {
    /// RPC listen address.
    #[structopt(short = "l", long)]
    rpc_listen: SocketAddr,
}

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    let opt = Opt::from_args();
    info!("rusty-workers-runtime starting");

    server::RuntimeServer::listen(&opt.rpc_listen, || server::RuntimeServer).await;

    Ok(())
}
