pub mod client;
pub mod crypto;
pub mod inout;
pub mod mesh_client;
pub mod proto;
pub mod service;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
pub struct Config {
    /// Path to the mesh key used to authenticate with other derp servers
    #[arg(long)]
    pub meshkey: Option<String>,

    /// List of other derp servers with which we should create a mesh
    #[arg(long)]
    pub mesh_peers: Vec<String>,

    #[arg(long, short)]
    pub listen_on: String,
}
