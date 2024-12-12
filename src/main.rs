use std::io::Write;
use std::{collections::HashMap, env, str::FromStr};

use chrono::Utc;
use config::Config;
use inbound::factory_consumer::get_consumer;
use outbound::{client_grpc, factory_writer::get_writer};
use service::factory_llm::get_llm;

mod inbound;
mod model;
mod outbound;
mod service;

fn main() {
    // read configuration
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("application requires first argument to be path of configuration file");
    }
    let config_path = &args[1];

    let config_data = match Config::builder()
        .add_source(config::File::with_name(config_path))
        .build()
    {
        Ok(v) => v,
        Err(e) => panic!("config error: {}", e),
    };

    let config: HashMap<String, HashMap<String, String>> =
        match config_data.try_deserialize::<HashMap<String, HashMap<String, String>>>() {
            Ok(v) => v,
            Err(e) => panic!("config error: {}", e),
        };

    // setting the logger
    let log_level = log::LevelFilter::from_str(config["log"]["level"].as_str()).unwrap();
    env_logger::builder()
        .filter_level(log_level)
        .format_target(false)
        .format_timestamp_secs()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} {} [{}:{}] {}",
                Utc::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.module_path().unwrap(),
                record.line().unwrap(),
                record.args()
            )
        })
        .init();

    // create writer object
    let mut writer_worker = match get_writer(&config["writer"]["type"]) {
        Ok(w) => w,
        Err(e) => panic!("writer error: {}", e),
    };
    writer_worker.init(&config);

    // create llm object
    let mut llm_worker = match get_llm(&config["llm"]["type"]) {
        Ok(l) => l,
        Err(e) => panic!("llm error: {}", e),
    };

    // create consumer object
    let mut consumer_worker = match get_consumer(&config["consumer"]["type"]) {
        Ok(c) => c,
        Err(e) => panic!("consumer error: {}", e),
    };

    // starting the application
    log::info!("application <<{}>> starting...", config["meta"]["id"]);
    consumer_worker.start(&config, &mut llm_worker, &mut writer_worker);
}

#[cfg(test)]
mod tests_main {
    use super::*;

    #[test]
    #[should_panic]
    fn test_main_without_config() {
        main();
    }
}
