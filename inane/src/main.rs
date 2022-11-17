use inane::{InaneInboundConfig, InaneOptions, InaneState};
use std::time::Instant;
use twelf::reexports::clap::CommandFactory;
use twelf::Layer;

const CONFIGS: &[&str] = &[
    "sitefix.json",
    "sitefix.yml",
    "sitefix.yaml",
    "sitefix.toml",
];

fn main() {
    let start = Instant::now();

    let matches = InaneInboundConfig::command().get_matches();

    let mut config_layers = vec![];

    let configs: Vec<&str> = CONFIGS
        .iter()
        .filter(|c| std::path::Path::new(c).exists())
        .cloned()
        .collect();
    if configs.len() > 1 {
        eprintln!(
            "Found multiple possible config files: [{}]",
            configs.join(", ")
        );
        eprintln!("Inane only supports loading one configuration file format, please ensure only one file exists.");
        std::process::exit(1);
    }

    for config in configs {
        let layer_fn = if config.ends_with("json") {
            Layer::Json
        } else if config.ends_with("toml") {
            Layer::Toml
        } else if config.ends_with("yaml") || config.ends_with("yml") {
            Layer::Yaml
        } else {
            eprintln!("Unknown config file format {}", config);
            std::process::exit(1);
        };
        config_layers.push(layer_fn(config.into()));
    }

    config_layers.push(Layer::Env(Some("INANE_".to_string())));
    config_layers.push(Layer::Clap(matches));

    match InaneInboundConfig::with_layers(&config_layers) {
        Ok(config) => {
            if let Ok(options) = InaneOptions::load(config.clone()) {
                let mut runner = InaneState::new(options);

                runner.run();

                let duration = start.elapsed();

                runner.options.logger.status(&format!(
                    "Finished in {}.{} seconds",
                    duration.as_secs(),
                    duration.subsec_millis()
                ));
            }
        }
        Err(e) => {
            eprintln!("Error loading Inane config:");
            match e {
                twelf::Error::Io(e) => {
                    eprintln!("{}", e);
                }
                twelf::Error::Envy(e) => {
                    eprintln!("{}", e);
                }
                twelf::Error::Json(e) => {
                    eprintln!("{}", e);
                }
                twelf::Error::Toml(e) => {
                    eprintln!("{}", e);
                }
                twelf::Error::Yaml(e) => {
                    eprintln!("{}", e);
                }
                twelf::Error::Deserialize(e) => {
                    eprintln!("{}", e);
                }
                _ => {
                    eprintln!("Unknown Error");
                }
            }
            std::process::exit(1);
        }
    }
}
