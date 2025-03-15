use clap::Parser;
use config::{Config, File};
use dirs::config_dir;
use once_cell::sync::Lazy;
use std::fs;
use std::net::{IpAddr, Ipv4Addr};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(long, short, default_value_t = 6379)]
    pub port: u16,

    #[arg(long, short, default_value_t = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)))]
    pub bind: IpAddr,
}

fn get_config_path() -> PathBuf {
    let base_dir = config_dir().unwrap_or_else(|| PathBuf::from("."));
    base_dir.join("zyst").join("config.toml")
}

fn create_default_config(config_path: &PathBuf) {
    let default_config = r#"
        [main]
        port = 6379
        bind = "127.0.0.1"
    "#;

    let parent = config_path
        .parent()
        .expect("Failed to get config directory");

    fs::create_dir_all(parent).expect("Failed to create config directory");
    fs::write(config_path, default_config).expect("Failed to write default config file");
}

static CONFIG: Lazy<Config> = Lazy::new(|| {
    let config_path = get_config_path();

    let cli = Cli::parse();

    if !config_path.exists() {
        println!(
            "Config file not found. Creating default at {:?}",
            config_path
        );
        create_default_config(&config_path);
    }

    Config::builder()
        .add_source(File::with_name(config_path.to_str().expect("No file")))
        .set_override("port", cli.port)
        .expect("Failed to set port override")
        .set_override("bind", cli.bind.to_string())
        .expect("Failed to set bind override")
        .build()
        .expect("Failed to load config")
});

pub fn get_config() -> &'static Config {
    &CONFIG
}
