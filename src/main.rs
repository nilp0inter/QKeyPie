mod config;
mod actions;
mod model;
mod controller;
mod events;
mod state;
mod server;

use clap::{Command, Arg};

fn cli() -> Command {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("qkeypie").unwrap();
    let config_file = xdg_dirs.get_config_file("config.toml");

    Command::new("qkeypie")
        .about("QKeyPie Daemon for the Xencelabs Quick Keys")
        .arg(Arg::new("CONFIG")
            .help("The configuration file")
            .short('c')
            .default_value(config_file.to_str().unwrap().to_string()))
        // .arg(arg!(--config <CONFIG> "The configuration file").short('c').default_value(config_file.to_str().unwrap().to_string()))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let matches = cli().get_matches();

    // start server::main() in a separate thread
    server::main().await;

    let controller = tokio::spawn(async move {
        // start the controller in the main thread
        let cfg = config::read_config(matches.get_one::<String>("CONFIG").unwrap())?;
        let model = model::from_config(cfg)?;
        controller::run(model)
    }); 

    // wait for the controller to finish
    controller.await??;

    Ok(())
}
    
