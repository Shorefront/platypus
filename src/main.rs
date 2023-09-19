
use log::info;

mod model;
mod template;

#[warn(missing_docs)]

fn main() {
    let pkg = env!("CARGO_PKG_NAME");
    let ver = env!("CARGO_PKG_VERSION");

    env_logger::init();

    info!("Starting {pkg} v{ver}");
}
