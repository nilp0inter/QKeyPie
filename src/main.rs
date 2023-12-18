mod config;
mod actions;
mod model;
mod controller;

fn main() -> anyhow::Result<()> {
    let cfg = config::read_config("config.toml")?;
    let model = model::from_config(cfg)?;
    controller::run(model)
}
    
