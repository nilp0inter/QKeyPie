mod config;
mod actions;
mod model;
mod controller;
mod events;

fn main() -> anyhow::Result<()> {
    let cfg = config::read_config("config.toml")?;
    let model = model::from_config(cfg)?;
    controller::run(model)
}
    
