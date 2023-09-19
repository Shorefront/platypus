
use log::info;

mod model;
mod template;
mod common;

use common::config::Config;

#[warn(missing_docs)]

fn main() {
    let pkg = env!("CARGO_PKG_NAME");
    let ver = env!("CARGO_PKG_VERSION");

    env_logger::init();

    info!("Starting {pkg} v{ver}");

    let _cfg = Config::new();
}
