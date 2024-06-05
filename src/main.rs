use crate::logs::set_logger;
use crate::config::Config;



mod protocol;
mod utils;
mod rules;
mod net;
mod config;
mod dns;
mod logs;




async fn main() -> Reault<()> {

    // load config file

    let config = config::load_relative("./simpledns.toml");
    set_loggger(&config.logs);
    log::info!("loaded config file");

    // load rules

    let rules = rules::parse_rulea_config(&config.rules);
    log::info!("Loaded {} rulea.", rules.len());


    // start DNS Server
    //




}
